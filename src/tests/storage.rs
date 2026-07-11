use crate::core::persistence::{AppPaths, Storage};

#[test]
fn test_atomic_write() 
{
    let path = &AppPaths::CacheFile{ filename:String::from("test_atomic_write") };
    let data = vec![b'y',b'u',b'y',b'a',b'n',b'\n'];

    let storage = Storage::new();

    if let Err(e) = storage.save(data, path)
    {
        panic!("error occured while data saving: {e:?}")
    };
}