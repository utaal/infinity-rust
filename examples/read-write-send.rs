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
        let buffer_token = buffer_to_read_write.region_token();

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
        unsafe {
            let mut context = ffi::infinity::core::Context::new(0, 1);
            let mut qp_factory = ffi::infinity::queues::QueuePairFactory::new(

            &mut context as *mut _);
		    eprintln!("Connecting to remote node");
            let qp = qp_factory.connectToRemoteHost(
                ::std::ffi::CString::new("192.168.1.62").unwrap().as_ptr(),
                8011,
                ::std::ptr::null_mut::<::std::os::raw::c_void>(),
                0);
            let remote_buffer_token = (*qp).getUserData() as (*mut ffi::infinity::memory::RegionToken);

            eprintln!("Creating buffers");
            let mut buffer_1_sided = ffi::infinity::memory::Buffer::new(
                &mut context as *mut _, 128);
            let mut buffer_2_sided = ffi::infinity::memory::Buffer::new(
                &mut context as *mut _, 128);

            eprintln!("Reading content from remote buffer");
            let mut request_token = ffi::infinity::requests::RequestToken::new(&mut context as *mut _);
            (*qp).read(&mut buffer_1_sided as *mut _, remote_buffer_token, &mut request_token as *mut _);
            request_token.waitUntilCompleted();

            let buffer_1_sided_data = ::std::mem::transmute::<_, &mut u64>(buffer_1_sided.getData());
            *buffer_1_sided_data = 84;

            eprintln!("Writing content to remote buffer");
            (*qp).write(&mut buffer_1_sided as *mut _, remote_buffer_token, &mut request_token as *mut _);
            request_token.waitUntilCompleted();

            let buffer_2_sided_data = ::std::mem::transmute::<_, &mut u64>(buffer_2_sided.getData());
            *buffer_2_sided_data = 42;

            eprintln!("Sending message to remote host");
            (*qp).send(&mut buffer_2_sided as *mut _, &mut request_token as *mut _);
            request_token.waitUntilCompleted();

            ffi::infinity::memory::Buffer_Buffer_destructor(&mut buffer_1_sided as *mut _);
            ffi::infinity::memory::Buffer_Buffer_destructor(&mut buffer_2_sided as *mut _);

            ffi::infinityhelpers::queues::delete_QueuePair(qp);

            qp_factory.destruct();
            context.destruct();
        }
    }
}
