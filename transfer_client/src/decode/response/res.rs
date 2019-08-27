use number_conv::array::u8arr;

#[macro_export]
macro_rules! decode_response {
    ($index:ident, $data:ident, $response:ident) => ({
        // println!("{:?}", $data);
        if $index % 2 == 0 {
            let mut number: u64 = 0;
            u8arr::u8arrTou64($data.as_slice(), &mut number);
            return (true, number);
        }
        if $index == 1 {
            $response.responseMode = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 3 {
            $response.serverUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 5 {
            let mut result = 0;
            u8arr::u8arrTou64($data.as_slice(), &mut result);
            $response.result = result as u8;
        } else if $index == 7 {
            $response.selfCommunicateUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 9 {
            $response.peerCommunicateUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 11 {
            $response.dataUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 13 {
            $response.objectUuid = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 15 {
            // println!("{:?}", &$data.as_slice());
            $response.peerResult = match String::from_utf8($data) {
                Ok(s) => s,
                Err(_) => "".to_string()
            };
        } else if $index == 17 {
            u8arr::u8arrTou64($data.as_slice(), &mut $response.u64Field1);
            // println!("{:?}", &$data.as_slice());
        } else if $index == 19 {
            u8arr::u8arrTou64($data.as_slice(), &mut $response.u64Field2);
            // println!("{:?}", &$data.as_slice());
        } else if $index == 21 {
            $response.data = $data;
        } else if $index == 23 {
            $response.extraData = $data;
        }
        if $index == 23 {
            return (false, 0);
        }
        if $index == 1 || $index == 3 || $index == 5 || $index == 7 || $index == 9 || $index == 11 || $index == 15 || $index == 17 {
            return (true, 1);
        } else {
            return (true, 4);
        }
    })
}
