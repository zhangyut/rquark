pub struct Task<T> {
    id: u64,
    user: String,
    cmds: Vec<T>,
}

impl<T> Task<T> {
    fn new(id: u64, user: String) -> Self {
        return Task {
            id: id,
            user: user,
            cmds: Vec::new(),
        };
    }
    fn add_cmd(&mut self, cmd: T) {
        self.cmds.push(cmd);
    }
    fn clear_cmd(&mut self) {
        self.cmds.clear();
    }
    fn at(&self, i: usize) -> &T {
        &self.cmds[i]
    }
    fn len(&self) -> usize {
        self.cmds.len()
    }
}

pub trait RawResult {
    fn to_string(&self) -> &String;
    fn to_json(&self) -> &String;
}

pub struct TaskResult<T: RawResult> {
    id: u64,
    code: u8,
    result: T,
}

impl<T: RawResult> TaskResult<T> {
    fn new(id: u64, code: u8, result: T) -> Self {
        return TaskResult {
            id: id,
            code: code,
            result: result,
        };
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
    fn result_to_string(&self) -> String {
        let mut s = String::from("");
        s.push_str(&*self.id.to_string());
        s.push_str("|");
        s.push_str(&*self.code.to_string());
        s.push_str("|");
        s.push_str(&*self.result.to_string());
        s
    }
}

pub struct EmptyResult {
    msg: String,
}
impl EmptyResult {
    fn new() -> Self {
        return EmptyResult {
            msg: String::from(""),
        };
    }
}
impl RawResult for EmptyResult {
    fn to_string(&self) -> &String {
        &self.msg
    }
    fn to_json(&self) -> &String {
        &self.msg
    }
}

fn succeed(id: u64) -> TaskResult<EmptyResult> {
    TaskResult::new(id, 200, EmptyResult::new())
}
fn succeed_with_result<T: RawResult>(id: u64, result: T) -> TaskResult<T> {
    TaskResult::new(id, 200, result)
}

#[cfg(test)]
mod test {
    use super::*;
    pub struct MyCmd {
        id: u64,
    }

    impl MyCmd {
        fn new(id: u64) -> Self {
            return MyCmd { id: id };
        }
        fn string(&self) -> String {
            let s = self.id.to_string();
            s
        }
    }

    pub struct MyResult {
        err_msg: String,
    }

    impl MyResult {
        fn new(msg: String) -> Self {
            return MyResult { err_msg: msg };
        }
    }

    impl RawResult for MyResult {
        fn to_string(&self) -> &String {
            &self.err_msg
        }
        fn to_json(&self) -> &String {
            &self.err_msg
        }
    }

    #[test]
    fn test_task() {
        let mut t: Task<MyCmd> = Task::new(12345, String::from("hello world"));
        let c = MyCmd::new(123456);
        t.add_cmd(c);
        assert_eq!(t.at(0).id, 123456);
        assert_eq!(t.at(0).id, 123456);
        let movetest = t.at(0).id;
        assert_eq!(movetest, 123456);
        assert_eq!(t.at(0).id, 123456);

        let ret = succeed(t.id);
        let mut ret_str = String::from("");
        ret_str.push_str(&*t.id.to_string());
        ret_str.push_str("|200|");
        assert_eq!(ret.result_to_string(), ret_str);
        assert_eq!(*ret.get_result().to_string(), String::from(""));

        let mut err_msg = String::from("我错了");
        assert_eq!(err_msg, String::from("我错了"));
        println!("before modify {:?}", err_msg);
        err_msg = String::from("我又错了");
        println!("after modify {:?}", err_msg);
        assert_eq!(err_msg, "我又错了");
    }
}