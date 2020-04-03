pub struct Task<'a, T> {
    id: u64,
    user: &'a str,
    cmds: Vec<T>,   
}

impl<'a, T> Task<'a, T> {
    fn new(id:u64, user:&'a str) -> Self {
        return Task{id:id, user:user, cmds:Vec::new()};
    }
    fn add_cmd(&mut self,cmd :T) {
        self.cmds.push(cmd);
    }
    fn clear_cmd(&mut self) {
        self.cmds.clear();
    }
    fn at(&self, i:usize) -> &T {
        &self.cmds[i]
    }
    fn len(&self) -> usize {
        self.cmds.len()
    }
}

pub struct MyCmd {
    id: u64,
}

impl MyCmd {
    fn new(id:u64) -> Self {
        return MyCmd{id:id};
    }
    fn string(&self) -> String {
        let s = self.id.to_string();
        s
    }
}

#[cfg(test)]
mod test {
    use super::*;
    pub struct MyCmd {
        id: u64,
    }

    impl MyCmd {
        fn new(id:u64) -> Self {
            return MyCmd{id:id};
        }
        fn string(&self) -> String {
            let s = self.id.to_string();
            s
        }
    }
    #[test]
    fn test_task() {
        let mut t:Task<MyCmd> = Task::new(12345, "hello world");
        let c = MyCmd::new(123456);
        t.add_cmd(c);
        assert_eq!(t.at(0).id, 123456);
        assert_eq!(t.at(0).id, 123456);
        let movetest = t.at(0).id;
        assert_eq!(movetest, 123456);
        assert_eq!(t.at(0).id, 123456);
    }
}