pub trait Command<T> {
    fn execute(&self, t: &mut T);
    fn undo(&self, t: &mut T);
}

pub struct Invoker<'a, Cmd, T: 'a> {
    commands: Vec<Cmd>,
    target: &'a mut T,
    current_index: usize,
}

impl<'a, Cmd, T>Invoker<'a, Cmd, T> {
    pub fn new(t: &'a mut T) -> Self {
        // Initialize
        Invoker {
            commands: Vec::new(),
            target: t,
            current_index: 0
        }
    }

    pub fn target(&self) -> &T {
        self.target
    }

    pub fn append_command(&mut self, c: Cmd) {
        self.commands.push(c);
    }
}

impl<'a, Cmd, T>Invoker<'a, Cmd, T> 
    where Cmd: Command<T>
{
    pub fn execute_command(&mut self) {
        if self.commands.len() <= self.current_index {
            // Nothing TO DO
            return;
        }

        let c = &self.commands[self.current_index]; // Get Command
        let t = &mut *self.target;  // Get Target
        c.execute(t);    // Execute

        self.current_index += 1;    // increment Index
    }

    pub fn execute_all_commands(&mut self) {
        // Loop Current Index to Commands Length
        for _ in self.current_index..self.commands.len() {
            self.execute_command();
        }
    }

    pub fn undo(&mut self) {
        if 0 == self.current_index {
            return;
        }

        self.current_index -= 1;

        let c = &self.commands[self.current_index];
        let t = &mut *self.target;

        c.undo(t);
    }
}
