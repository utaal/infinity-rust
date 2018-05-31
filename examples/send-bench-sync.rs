extern crate infinity;

use std::time::Instant;

// ------ Helpers ----------

mod helpers {
    use libc::{_SC_PAGESIZE, sysconf};

    #[inline]
    pub fn get_page_size() -> usize {
        unsafe {
            sysconf(_SC_PAGESIZE) as usize
        }
    }
}

// ------ HDHistogram ------

const HDHISTOGRAM_BITS: usize = 4;

#[derive(Clone, Debug)]
struct HDHistogram {
    counts: Vec<[u64; 1 << HDHISTOGRAM_BITS]>,
}

impl HDHistogram {
    pub fn add_value(&mut self, value: u64) {
        let index = value.next_power_of_two().trailing_zeros() as usize;
        let low_bits = (value >> (index - HDHISTOGRAM_BITS - 1)) & ((1 << HDHISTOGRAM_BITS) - 1);
        self.counts[index][low_bits as usize] += 1;
    }

    pub fn flatten(&self) -> Vec<(u64, f64, u64)> {
        let mut results = Vec::new();
        let total = self.counts.iter().map(|x| x.iter().sum::<u64>()).sum();
        let mut sum = 0;
        for index in (0 .. self.counts.len()).rev() {
            for sub in (0 .. (1 << HDHISTOGRAM_BITS)).rev() {
                if sum > 0 && sum < total && self.counts[index][sub] > 0 {
                    let latency = (1 << (index - 1)) + (sub << (index - HDHISTOGRAM_BITS - 1));
                    let fraction = (sum as f64) / (total as f64);
                    results.push((latency as u64, fraction, self.counts[index][sub]));
                }
                sum += self.counts[index][sub];
            }
        }
        results.reverse();
        results
    }
}

impl Default for HDHistogram {
    fn default() -> Self {
        HDHistogram {
            counts: vec![[0u64; 16]; 64],
        }
    }
}

// -------------------------

fn main() {
    let mut args = ::std::env::args();
    args.next().unwrap();

    let server = match args.next().unwrap().as_str() {
        "server" => true,
        "client" => false,
        _ => panic!("invalid mode"),
    };

    let rounds: u64 = args.next().unwrap().parse().unwrap();

    if server {
        let context = infinity::core::Context::new(0, 1);
        let mut qp_factory = infinity::queues::QueuePairFactory::new(&context);

        eprintln!("Creating and posting buffers to receive a message");
        let mut posted_buffers = 0;
        for _ in 0..64 {
            let buffer = infinity::memory::Buffer::new(&context, 128);
            context.post_receive_buffer(buffer);
            posted_buffers += 1;
        }

        eprintln!("Setting up connection (blocking)");
        qp_factory.bind_to_port(8011);
        let _qp = qp_factory.accept_incoming_connection(&[]);

        eprintln!("Receiving messages");
        for i in 1..=rounds {
            let infinity::core::ReceiveElement { buffer: (mut recv_buf, _recv_len), immediate: _, } = loop {
                let el = context.receive();
                if let Some(el) = el {
                    break el;
                }
            };
            posted_buffers -= 1;

            unsafe {
                let receive_element_data = ::std::mem::transmute::<_, &mut u64>((&mut recv_buf[..]).as_mut_ptr());
                if *receive_element_data != i {
                    eprintln!("Incorrect data: {} != {}", *receive_element_data, i);
                }
            }

            if posted_buffers < rounds - i {
                context.post_receive_buffer(recv_buf);
                posted_buffers += 1;
            }

            if i % 100000 == 0 {
                eprintln!("Round {} done", i);
            }
        }
        eprintln!("End, posted buffers: {}", posted_buffers);
    } else {
        let context = infinity::core::Context::new(0, 1);
        let mut qp_factory = infinity::queues::QueuePairFactory::new(&context);

        eprintln!("Connecting to remote node");
        let mut qp = qp_factory.connect_to_remote_host(::std::net::SocketAddr::from(([192, 168, 1, 62], 8011)), &[]);

        eprintln!("Creating buffer");
        let mut buffer_2_sided = Some(infinity::memory::Buffer::new(&context, 128));

        let mut hd_hist: HDHistogram = Default::default();

        eprintln!("Sending messages");
        let start = Instant::now();
        for i in 1..=rounds {
            let prev = Instant::now();
            unsafe {
                let buffer_2_sided_data = ::std::mem::transmute::<_, &mut u64>(
                    (&mut buffer_2_sided.as_mut().unwrap()[..]).as_mut_ptr());
                *buffer_2_sided_data = i;
            }

            let request_token = qp.send(buffer_2_sided.take().unwrap());
            buffer_2_sided = Some(request_token.wait_until_completed().expect("Send failed").buffer);

            let rtt_elapsed = prev.elapsed();
            hd_hist.add_value(rtt_elapsed.as_secs() * 1_000_000_000 + rtt_elapsed.subsec_nanos() as u64);
        }
        let end = start.elapsed();
        eprintln!("End: {:?}", end);
        let nanos = end.as_secs() * 1_000_000_000 + end.subsec_nanos() as u64;
        println!("RTT\t{}", nanos as f64 / rounds as f64);
        for (l, f, c) in hd_hist.flatten().into_iter() {
            println!("CCDF\t{}\t{}\t{}", l, f, c);
        }
    }
}

