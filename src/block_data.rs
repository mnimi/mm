pub struct Block{

    id : u32,
    tags : Vec<u32>

}

pub struct Chunk{

    block : [u32 ; 4096]

}
impl Chunk {

    pub fn index(x : u8, y : u8, z : u8) -> u32 {

        (x + (y * 16) + (z * 256)) as u32

    }

}


pub struct Map{

    chunk : Vec<Chunk>,
    x : Vec<i32>,
    y : Vec<i32>,
    z : Vec<i32>

}
impl Map{

    pub fn get_chunk(&self, x : i32, y : i32, z : i32) -> Chunk{

        const X : i32 = (x - (x % 16)) / 16;
        const Y : i32 = (x - (x % 16)) / 16;
        const Z : i32 = (x - (x % 16)) / 16;

        let value = for i in 0..self.x.len(){

            if self.x[i] == X
                && self.y[i] == Y
                && self.z[i] == Z
            {

                break Some(self.chunk[i]);

            }
        };

        match value {

            Some(v) => return v,
            () => return load_chunk(x, y, z)

        }

    }

    fn load_chunk(x : i32, y : i32, z : i32) -> Chunk{

        todo!()

    }

}