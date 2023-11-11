pub mod ping;

// pub struct GeneralCommands;

// impl ContextCommandCategory for GeneralCommands {
//     fn name(&self) -> &'static str {
//         "General Information"
//     }

//     fn collect_commands(&self) -> HashMap<&'static str, Box<dyn super::ContextCommandHandler>> {
//         let mut commands: HashMap<&'static str, Box<dyn ContextCommandHandler>> = HashMap::new();

//         let pingcommand = PingCommand {};
//         commands.insert(pingcommand.name(), Box::new(PingCommand));

//         commands
//     }
// }
