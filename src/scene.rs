pub mod scene {

    use std::sync::{Mutex, MutexGuard};

    #[derive(Debug)]
    pub struct Scene {
        val: u32,
    }


    impl Scene {
        fn access() -> MutexGuard<'static, Scene> {
            static GLOBSTATE: Mutex<Scene> = Mutex::new(Scene { val:0 });
            GLOBSTATE.lock().unwrap()
        }

        pub fn print() {
            let scene = Self::access();
            println!("{:?}", scene);
        }

        pub fn incr() {
            let mut scene = Self::access();
            scene.val += 1;
        }
    }
}