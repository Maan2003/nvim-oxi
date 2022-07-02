use nvim_oxi::{
    api::{self, Buffer},
    opts::*,
    types::*,
    LuaFun,
};

pub fn attach() {
    let buf = Buffer::current();

    let opts = BufAttachOpts::builder()
        .on_lines(|_args| Ok(false))
        .on_bytes(|_args| Ok(false))
        .on_detach(|_args| Ok(false))
        .on_reload(|_args| Ok(false))
        .on_changedtick(|_args| Ok(false))
        .build();

    let has_attached = buf.attach(false, &opts).expect("attach failed");
    assert!(has_attached);

    let bytes_written = api::input("ifoo<Esc>");
    assert!(bytes_written.is_ok(), "{bytes_written:?}");
}

pub fn call() {
    let buf = Buffer::current();
    let res = buf.call(|_| Ok(()));
    assert_eq!(Ok(()), res);
}

pub fn create_del_user_command() {
    let mut buf = Buffer::current();
    let opts = CreateCommandOpts::builder().build();

    let res = buf.create_user_command("Foo", ":", &opts);
    assert_eq!(Ok(()), res);
    api::command("Foo").unwrap();

    let cb = LuaFun::from_fn(|_args: CommandArgs| Ok(()));
    let res = buf.create_user_command("Bar", cb, &opts);
    assert_eq!(Ok(()), res);
    api::command("Bar").unwrap();

    let opts = GetCommandsOpts::builder().build();
    assert_eq!(2, buf.get_commands(&opts).unwrap().collect::<Vec<_>>().len());

    assert_eq!(Ok(()), buf.del_user_command("Foo"));
    assert_eq!(Ok(()), buf.del_user_command("Bar"));
}

pub fn get_changedtick() {
    let buf = Buffer::current();
    assert!(buf.get_changedtick().is_ok());
}

pub fn loaded_n_valid() {
    let buf = Buffer::current();
    assert!(buf.is_loaded());
    assert!(buf.is_valid());
}

pub fn new_buf_delete() {
    let buf = api::create_buf(true, false).unwrap();
    let opts = BufDeleteOpts::builder().build();
    assert_eq!(Ok(()), buf.delete(&opts));
}

pub fn set_get_del_keymap() {
    let mut buf = Buffer::current();

    let opts = SetKeymapOpts::builder()
        .callback(|_| Ok(()))
        .desc("does nothing")
        .expr(true)
        .build();

    let res = buf.set_keymap(Mode::Insert, "a", None, &opts);
    assert_eq!(Ok(()), res);

    let keymaps = buf.get_keymap(Mode::Insert).unwrap().collect::<Vec<_>>();
    assert_eq!(1, keymaps.len());

    let res = buf.del_keymap(Mode::Insert, "a");
    assert_eq!(Ok(()), res);
}

pub fn set_get_del_lines() {
    let mut buf = Buffer::current();

    assert_eq!(Ok(()), buf.set_lines(0, 0, true, ["foo", "bar", "baz"]));
    assert_eq!(
        vec!["foo", "bar", "baz", ""],
        buf.get_lines(0, 4, true)
            .unwrap()
            .flat_map(TryFrom::try_from)
            .collect::<Vec<String>>()
    );
    assert_eq!(Ok(4), buf.line_count());

    assert_eq!(Ok(()), buf.set_lines::<String, _>(0, 4, true, []));
    assert_eq!(Ok(1), buf.line_count());
}

pub fn set_get_del_mark() {
    let mut buf = Buffer::current();

    let res = buf.set_mark('a', 1, 0);
    assert_eq!(Ok(true), res);

    assert_eq!((1, 0), buf.get_mark('a').unwrap());

    let res = buf.del_mark('a');
    assert_eq!(Ok(true), res);
}

pub fn set_get_del_text() {
    let mut buf = Buffer::current();

    assert_eq!(Ok(()), buf.set_text(0, 0, 0, 0, ["foo", "bar", "baz"]));
    assert_eq!(
        vec!["foo", "bar", "baz"],
        buf.get_text(0, 0, 2, 3)
            .unwrap()
            .flat_map(TryFrom::try_from)
            .collect::<Vec<String>>()
    );
    assert_eq!(Ok(3), buf.line_count());

    assert_eq!(Ok(()), buf.set_text::<String, _>(0, 0, 2, 3, []));

    // Please someone explain to me how these two give different lengths
    // because evidently I'm too fucking retarded to get it.
    assert_eq!(
        0,
        buf.get_text(0, 0, 0, 1)
            .unwrap()
            .map(String::try_from)
            .collect::<Result<Vec<_>, _>>()
            .unwrap()
            .len()
    );

    assert_eq!(
        1,
        buf.get_text(0, 0, 0, 1)
            .unwrap()
            .map(String::try_from)
            .collect::<Vec<Result<_, _>>>()
            .len()
    );

    assert_eq!(Ok(1), buf.line_count());
}

pub fn set_get_del_var() {
    let mut buf = Buffer::current();
    buf.set_var("foo", 42).unwrap();
    assert_eq!(Ok(42), buf.get_var("foo"));
    assert_eq!(Ok(()), buf.del_var("foo"));
}

pub fn set_get_name() {
    let mut buf = Buffer::current();

    assert_eq!("", buf.get_name().unwrap().display().to_string());

    assert_eq!(Ok(()), buf.set_name("foo"));

    assert_eq!(
        "foo",
        buf.get_name().unwrap().file_name().unwrap().to_string_lossy()
    );

    assert_eq!(Ok(()), buf.set_name(""));
}

pub fn set_get_option() {
    let mut buf = Buffer::current();

    buf.set_option("modified", true).unwrap();
    assert!(buf.get_option::<bool>("modified").unwrap());

    buf.set_option("modified", false).unwrap();
    assert!(!buf.get_option::<bool>("modified").unwrap());
}
