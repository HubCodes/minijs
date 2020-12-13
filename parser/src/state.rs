pub struct State {
    symbol_id: i32,
    scope_id: i32,
}

impl State {
    pub fn new() -> State {
        State { symbol_id: 0, scope_id: 0 }
    }

    pub fn next_symbol_id(&mut self) -> i32 {
        let next_id = self.symbol_id;
        self.symbol_id += 1;
        next_id
    }

    pub fn next_scope_id(&mut self) -> i32 {
        let next_id = self.scope_id;
        self.scope_id += 1;
        next_id
    }
}
