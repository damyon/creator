


pub mod command_queue {
    use crate::command::command::Command;
    use crate::command::command::CommandType;

    pub struct CommandQueue {
        pub pending_count: usize,
        pub current_index: usize,
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

        pub fn queue_command(&mut self, command: Command) {
            if self.pending_count < self.commands.len() {
                self.commands[self.current_index] = command;
                self.current_index = (self.current_index + 1) % self.commands.len();
                self.pending_count += 1;
            }
        }

        pub fn next_command(&mut self) -> Option<Command> {
            if self.pending_count == 0 {
                None
            } else {
                self.pending_count = self.pending_count - 1;
                let return_index = self.current_index;
                self.current_index = (self.current_index + 1) % self.commands.len();
                Some(self.commands[return_index])
            }
        }
    }

}