extern crate infinity;

use infinity::ffi;

fn main() {
    let mut args = ::std::env::args();
    args.next().unwrap();

    let server = match args.next().unwrap().as_str() {
        "server" => true,
        "client" => false,
        _ => panic!("invalid mode"),
    };

    unsafe {
        let mut context = ffi::infinity::core::Context::new(0, 1);
        let mut qp_factory = ffi::infinity::queues::QueuePairFactory::new(
            &mut context as *mut _);

        if server {
            eprintln!("Creating buffers to read from and write to");
            let mut buffer_to_read_write = ffi::infinity::memory::Buffer::new(
                &mut context as *mut _, 128);
            let buffer_to_read_write_ptr: *mut _ = &mut buffer_to_read_write as *mut _;
            let buffer_token = 
                (*::std::mem::transmute::<_, *mut ffi::infinity::memory::Region>(buffer_to_read_write_ptr)).createRegionToken();

            eprintln!("Creating buffers to receive a message");
            let mut buffer_to_receive = ffi::infinity::memory::Buffer::new(
                &mut context as *mut _, 128);
            context.postReceiveBuffer(&mut buffer_to_receive as *mut _);

            eprintln!("Setting up connection (blocking)");
            qp_factory.bindToPort(8011);
            let qp = qp_factory.acceptIncomingConnection(
                buffer_token as *mut ::std::os::raw::c_void,
                ::std::mem::size_of::<ffi::infinity::memory::RegionToken>() as u32);

            eprintln!("Waiting for message (blocking)");
            let mut receive_element: ffi::infinity::core::receive_element_t = ::std::mem::zeroed();
            while !context.receive(&mut receive_element as *mut _) { }

            let receive_element_data = ::std::mem::transmute::<_, &mut u64>((*receive_element.buffer).getData());
            eprintln!("Message received: {}", receive_element_data);

            ffi::infinity::memory::Buffer_Buffer_destructor(&mut buffer_to_read_write as *mut _);
            ffi::infinity::memory::Buffer_Buffer_destructor(&mut buffer_to_receive as *mut _);
            (*qp).destruct();
        } else {
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
            (*qp).destruct();
        }

        qp_factory.destruct();
        context.destruct();
    }
}
