#[macro_export]
macro_rules! get_or_return {
    ($var:expr) => {
        match $var {
            Some(val) => val,
            None => return,
        }
    };
}

#[macro_export]
macro_rules! get_or_return_val {
    ($var:expr, $ret:expr) => {
        match $var {
            Some(val) => val,
            None => return $ret,
        }
    };
}

#[macro_export]
macro_rules! get_or_continue {
    ($var:expr) => {
        match $var {
            Some(val) => val,
            None => continue,
        }
    };
}

#[macro_export]
macro_rules! get_ok_or_return {
    ($var:expr) => {
        match $var {
            Ok(val) => val,
            Err(_) => return,
        }
    };
}

#[macro_export]
macro_rules! get_ok_or_continue {
    ($var:expr) => {
        match $var {
            Ok(val) => val,
            Err(_) => continue,
        }
    };
}
