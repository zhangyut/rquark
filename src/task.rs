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

pub trait RawResult {
    fn to_str(&self) -> &str;
    fn to_json(&self) -> &str;
}

pub struct TaskResult<T:RawResult> {
    id :u64,
    code:u8,
    result:T,
}

impl<T:RawResult> TaskResult<T> {
    fn new(id:u64, code:u8, result:T) -> Self {
        return TaskResult{id:id, code:code, result:result};
    }
    fn get_id(&self) -> &u64 {
        &self.id
    }
    fn get_code(&self) -> &u8 {
        &self.code
    }
    fn get_result(&self) -> &T {
        &self.result
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

    pub struct MyResult<'a> {
        err_msg : &'a str,
    }

    impl<'a> MyResult<'a> {
        fn new(msg:&'a str) -> Self {
            return MyResult{err_msg:msg};
        }
    }

    impl<'a> RawResult for MyResult<'a> {
        fn to_str(&self) -> &str {
            &self.err_msg 
        }
        fn to_json(&self) -> &str {
            &self.err_msg 
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

        let mut err_msg = "我错了";
        let myRet = MyResult::new(err_msg);
        assert_eq!(err_msg, "我错了");
        let ret = TaskResult::new(12345, 200, myRet);
        assert_eq!(ret.get_result().to_str(), "我错了");
        err_msg = "我又错了";
        assert_eq!(err_msg, "我又错了");
        assert_eq!(ret.get_result().to_str(), "我错了");
    }
}