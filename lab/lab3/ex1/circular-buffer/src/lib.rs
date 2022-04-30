
#[cfg(feature="version1")]
pub struct CircularBuffer<T: Clone + Default> {
    buffer: Vec<T>,
    capacity: usize,
    read_idx: usize,
    write_idx: usize,
    size: usize
}

#[cfg(feature="version2")]
pub struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    read_idx: usize,
    write_idx: usize
}

#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

#[cfg(feature="version1")]
impl<T: Clone + Default> CircularBuffer<T> {
    
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: (0..capacity).map(|_| T::default()).collect(), //init buffer with default value of Option with len=capacity
            capacity: capacity,
            read_idx: 0,
            write_idx: 0,
            size: 0 //size to save the number of elemenent in buffer
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        match self.capacity == self.size {
            // the buffer is full
            true => Err(Error::FullBuffer),
            
            false => {
                self.overwrite(_element);
                return Ok(())
            }
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.size == 0 {
            true => Err(Error::EmptyBuffer),
            false => {
                let ret = self.buffer[self.read_idx].clone(); //clone the element
                self.read_idx = (self.read_idx+1) % self.capacity; //incremet read index 
                self.size-=1; //decrement the number of element in the buffer
                return Ok(ret)
            }
        }
    }

    pub fn clear(&mut self) {
        self.buffer = (0..self.capacity).map(|_| T::default()).collect(); //init the buffer
        self.read_idx = 0;
        self.write_idx = 0;
        self.size = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        self.buffer[self.write_idx] = _element;
        self.write_idx = (self.write_idx + 1) % self.capacity;
        
        // check if it is an owerwrite or a simple write
        match self.size == self.capacity {
            true => self.read_idx = (self.read_idx+1) % self.capacity, //overwrite: the read idx increased
            false => self.size+=1 //simple write:the number of element in the buffer increased
        }
    }
}

#[cfg(feature="version2")]
impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        Self {
            buffer: (0..capacity).map(|_| Option::default()).collect(), //init buffer with default value of Option with len=capacity
            capacity: capacity,
            read_idx: 0,
            write_idx: 0
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        match self.buffer[self.write_idx].is_some() {
            true => Err(Error::FullBuffer), // if is Some(T) return fullBuffer error
            false => { // if is None write the  element 
                self.overwrite(_element);
                Ok(())
            }
        }  
    }

    pub fn read(&mut self) -> Result<T, Error> {
        match self.buffer[self.read_idx].take(){ //take the value and put default value (None) 
            // if it is Some(T) then a read_ind is increased and value is returned in a Result 
            Some(value) => {
                self.read_idx = (self.read_idx+1) % self.capacity; //incremet read index 
                Ok(value)
            }
            // if it is None the buffer is Empty and it is returned a EmptyBuffer error in a Result 
            None => Err(Error::EmptyBuffer)
        }
    }

    pub fn clear(&mut self) {
        self.buffer = (0..self.capacity).map(|_| Option::default()).collect(); //init the buffer
        self.read_idx = 0;
        self.write_idx = 0;
    }

    pub fn overwrite(&mut self, _element: T) {
        let mut flag_overwrite = false;
        if self.buffer[self.write_idx].is_some(){ //check if in the write position there is Some or None
            flag_overwrite = true;
        } 
        self.buffer[self.write_idx] = Some(_element);
        self.write_idx = (self.write_idx + 1) % self.capacity;
        if flag_overwrite{ // if it is an overwrite then the read index must be incresed
            self.read_idx = (self.read_idx+1) % self.capacity;
        }
    }
}
