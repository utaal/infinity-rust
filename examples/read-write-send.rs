extern crate infinity;

use std::io::Read;

use infinity::ffi;

fn main() {
    let mut args = ::std::env::args();
    args.next().unwrap();

    let server = match args.next().unwrap().as_str() {
        "server" => true,
        "client" => false,
        _ => panic!("invalid mode"),
    };

    if server {
        let mut context = infinity::core::Context::new(0, 1);
        let mut qp_factory = infinity::queues::QueuePairFactory::new(&context);

        eprintln!("Creating buffers to read from and write to");
        let buffer_to_read_write = infinity::memory::Buffer::new(&context, 128);
        let (_, buffer_token) = buffer_to_read_write.region_token();

        eprintln!("Creating buffers to receive a message");
        let mut buffer_to_receive = infinity::memory::Buffer::new(&context, 128);
        context.post_receive_buffer(buffer_to_receive);

        eprintln!("Setting up connection (blocking)");
        qp_factory.bind_to_port(8011);
        let _qp = qp_factory.accept_incoming_connection(buffer_token.as_bytes());

        eprintln!("Waiting for message (blocking)");
        let infinity::core::ReceiveElement { buffer: (mut recv_buf, recv_len), immediate, } = loop {
            let el = context.receive();
            if let Some(el) = el {
                break el;
            }
        };

        unsafe {
            let receive_element_data = ::std::mem::transmute::<_, &mut u64>((&mut recv_buf[..]).as_mut_ptr());
            eprintln!("Message received: {}", receive_element_data);
        }
    } else {
        let mut context = infinity::core::Context::new(0, 1);
        let mut qp_factory = infinity::queues::QueuePairFactory::new(&context);

        eprintln!("Connecting to remote node");
        let mut qp = qp_factory.connect_to_remote_host(::std::net::SocketAddr::from(([192, 168, 1, 62], 8011)), &[]);
        let remote_buffer_token = infinity::memory::RegionToken::from_bytes(qp.get_user_data());

        eprintln!("Creating buffers");
        // let mut buffer_1_sided = infinity::memory::Buffer::new(&context, 128);
        let mut buffer_2_sided = infinity::memory::Buffer::new(&context, 128);

        // skipping one-sided ops

        unsafe {
            let buffer_2_sided_data = ::std::mem::transmute::<_, &mut u64>((&mut buffer_2_sided[..]).as_mut_ptr());
            *buffer_2_sided_data = 42;
        }

        eprintln!("Sending message to remote host");
        let request_token = qp.send(buffer_2_sided);
        request_token.wait_until_completed();
    }
}

// eprintln!("Reading content from remote buffer");
// let mut request_token = ffi::infinity::requests::RequestToken::new(&mut context as *mut _);
// (*qp).read(&mut buffer_1_sided as *mut _, remote_buffer_token, &mut request_token as *mut _);
// request_token.waitUntilCompleted();

// let buffer_1_sided_data = ::std::mem::transmute::<_, &mut u64>(buffer_1_sided.getData());
// *buffer_1_sided_data = 84;

// eprintln!("Writing content to remote buffer");
// (*qp).write(&mut buffer_1_sided as *mut _, remote_buffer_token, &mut request_token as *mut _);
// request_token.waitUntilCompleted();
