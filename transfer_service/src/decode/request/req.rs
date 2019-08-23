#[macro_export]
macro_rules! decode_request {
    ($index:ident, $data:ident, $request:ident) => ({
        if $index % 2 == 0 {
            let mut number: u64 = 0;
            u8arr::u8arrTou64($data.as_slice(), &mut number);
            return (true, number);
        }
        if $index == 1 {
            $request.requestMode = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 3 {
            $request.selfCommunicateUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 5 {
            $request.peerCommunicateUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 7 {
            $request.serverUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 9 {
            $request.objectUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 11 {
            $request.peerResult = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 13 {
            u8arr::u8arrTou64($data.as_slice(), &mut $request.packageIndex);
        } else if $index == 15 {
            u8arr::u8arrTou64($data.as_slice(), &mut $request.packageTotal);
        } else if $index == 17 {
            $request.data = $data;
        } else if $index == 19 {
            $request.extraData = $data;
        }
        if $index == 19 {
            return (false, 0);
        }
        if $index == 1 || $index == 3 || $index == 5 || $index == 7 || $index == 11 || $index == 13 {
            return (true, 1);
        } else {
            return (true, 4);
        }
    })
}
