pub mod command {

    #[derive(Copy, Clone)]
    pub enum CommandType {
        // An `enum` variant may either be `unit-like`,
        MouseMoved,
        MouseDown,
        MouseUp,
        KeyDown
    }

    #[derive(Copy, Clone)]
    pub struct Command {
        pub command_type: CommandType,
        pub data1: u32,
        pub data2: u32
    }

}