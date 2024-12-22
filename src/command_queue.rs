


pub mod command_queue {
    use crate::command::command::Command;
    use crate::command::command::CommandType;

    pub struct CommandQueue {
        pub pending_count: u8,
        pub current_index: u8,
        pub commands: [Command; 10]
    }

    impl CommandQueue {

        pub const fn new() -> CommandQueue {
            CommandQueue {
                pending_count: 0,
                current_index: 0,
                commands: [
                    Command{
                        command_type: CommandType::MouseMoved,
                        data1: 0,
                        data2: 0
                    }; 10]
            }
        }
    }

}