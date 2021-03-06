use cocoa_foundation::base::id;
use objc::{class, msg_send, sel, sel_impl};

pub fn is_main_thread() -> bool {
    unsafe { msg_send![class!(NSThread), isMainThread] }
}

pub fn activate_cocoa_multithreading() {
    unsafe {
        let thread: id = msg_send![class!(NSThread), new];
        let _: () = msg_send![thread, start];
    }
}

pub struct NSApplication(pub id);

impl NSApplication {
    pub fn shared_application() -> Self {
        Self(unsafe { msg_send![class!(NSApplication), sharedApplication] })
    }

    pub fn is_running(&self) -> bool {
        unsafe { msg_send![self.0, isRunning] }
    }

    pub fn key_window(&self) -> id {
        unsafe { msg_send![self.0, keyWindow] }
    }
}

pub fn run_on_main<R: Send, F: FnOnce() -> R + Send>(run: F) -> R {
    if is_main_thread() {
        run()
    } else {
        let app = NSApplication::shared_application();
        if app.is_running() {
            let main = dispatch::Queue::main();
            main.exec_sync(run)
        } else {
            panic!("You are running RFD in NonWindowed environment, it is impossible to spawn dialog from thread different than main in this env.");
        }
    }
}
