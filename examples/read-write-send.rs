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
        let mut buffer_to_read_write = infinity::memory::Buffer::new(&context, 128);
        unsafe {
            let read_write_data = ::std::mem::transmute::<_, &mut u64>((&mut buffer_to_read_write[..]).as_mut_ptr());
            *read_write_data = 21;
        }
        let (mut buffer_to_read_write, buffer_token) = buffer_to_read_write.region_token();

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

            let buffer_to_read_write_data = ::std::mem::transmute::<_, &u64>((&buffer_to_read_write.read()[..]).as_ptr());
            eprintln!("1-sided buffer contains: {}", receive_element_data);
        }
    } else {
        let mut context = infinity::core::Context::new(0, 1);
        let mut qp_factory = infinity::queues::QueuePairFactory::new(&context);

        eprintln!("Connecting to remote node");
        let mut qp = qp_factory.connect_to_remote_host(::std::net::SocketAddr::from(([192, 168, 1, 62], 8011)), &[]);
        let remote_buffer_token = infinity::memory::RegionToken::from_bytes(qp.get_user_data()).expect("invalid remote buffer token");

        eprintln!("Creating buffers");
        let mut buffer_1_sided = infinity::memory::Buffer::new(&context, 128);
        let mut buffer_2_sided = infinity::memory::Buffer::new(&context, 128);

        eprintln!("Reading content from remote buffer");
        let request_token = qp.read(buffer_1_sided, &remote_buffer_token, Default::default());
        let infinity::requests::RequestResult { buffer: mut buffer_1_sided, .. } =
            request_token.wait_until_completed().expect("Read failed");

        unsafe {
            let buffer_1_sided_data = ::std::mem::transmute::<_, &mut u64>((&mut buffer_1_sided[..]).as_mut_ptr());
            eprintln!("Data read: {}", *buffer_1_sided_data);
            *buffer_1_sided_data *= 2;
        }

		eprintln!("Writing content to remote buffer");
        let request_token = qp.write(buffer_1_sided, &remote_buffer_token, Default::default());
        request_token.wait_until_completed().expect("Write failed");

        unsafe {
            let buffer_2_sided_data = ::std::mem::transmute::<_, &mut u64>((&mut buffer_2_sided[..]).as_mut_ptr());
            *buffer_2_sided_data = 42;
        }

        eprintln!("Sending message to remote host");
        let request_token = qp.send(buffer_2_sided, Default::default());
        request_token.wait_until_completed().expect("Send failed");
    }
}

