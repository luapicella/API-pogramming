pub fn find(array: &[i32], key: i32) -> Option<usize> {
    //base case
    let len = array.len();
    if len == 0 { return None }
    else if len == 1{
        match array[0] == key {
            true => Some(0),
            false => None
        }
    //divide et impera
    }else{
        let (left, right) = array.split_at(len/2);
        if key >= array[len/2]{
            let found = find(right, key);
            match found {
                None => None,
                Some(n) => Some(len/2+n)
            }
        }else{
            find(left, key)
        }        
    }
}

pub fn find2(array: &[i32], key: i32) -> Option<usize> {
    
    if array.len() == 0 {
        return None
    }
    
    let mut hi = array.len()-1;
    let mut lo = 0;

    if key < array[lo] || key > array[hi]{
        return None
    }
    
    while lo <= hi {
        let mid_id = ((hi - lo) / 2) + lo;
        let val = array[mid_id as usize];
        
        if val == key{
            return Some(mid_id as usize)
        }

        else if val > key {
            hi = mid_id - 1
        }

        else if val < key {
            lo = mid_id + 1
        }

    }
    None
}


