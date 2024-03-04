# Array<T>

- [Definition](#definition)

## Definition

```rs
/// A resizable and movable array type.
///
/// #[example]
/// ```qs
/// let mut arr: Array<i32> = Array::new();
///
/// // Add an item into the array.
/// arr.push(1); // arr = [1]
/// arr += [2, 3, 4]; // arr = [1, 2, 3, 4]
/// arr = arr + 3; // arr = [1, 2, 3, 4, 3]
///
/// // Remove an item from the array.
/// arr.pop(1); // arr = [1, 3, 4, 3]
/// arr -= 3; // arr = [1, 3, 4]
/// arr = arr - 1; // arr = [1, 4]
///
/// // Get an item in the array.
/// let item = arr.get(0); // item = 1
/// let item = arr[1]; // item = 4
/// ```
///
/// == Serialization Info
///
/// An array is serialized into an array of bytes
/// on the stack. Here's an example:
///
/// QuickScript:
/// ```qs
/// let mut arr: Array<i32> = Array::of(1, 2, 3, 4, 5);
/// ```
///
/// The stack:
/// ```
/// [Length, Item Size, Endianness, Spacer, Items...                     ]
/// [5,      4,         0,          0,      1, 2, 3, 4, 5                ]
/// [0x5,    0x4,       0x0,        0x0,    0x1, 0x2, 0x3, 0x4, 0x5      ]
/// [0b101,  0b100,     0b0,        0b0,    0b1, 0b10, 0b11, 0b100, 0b101]
/// [u32,    u32,       u8,         u8,     ]
/// ```
///
/// A note on endianness:
/// Endianness is represented as 0 being little, and 1 being big.
@Since(0.1, status = Status::Stable, feat = "arrays")
@Apply(Format, Clone, Sized)
@Cond(T: Copy, @Apply(Copy))
@Cond(T: Sync, @Apply(Sync))
pub struct Array<T, A> where T: Sized, A: Allocator {
    mut len: u32,
    mut base_ptr: u32,
    mut alloc: A,
}

@Since(0.1, status = Status::Stable, feat = "arrays")
impl<T, A> Array<T, A> where T: Sized, A: Allocator {
    @Constructor
    pub fn new() -> Self {
        let alloc = A::get();

        Self {
            len: 0,
            base_ptr: alloc.get_ptr(),
            alloc,
        }
    }

    pub fn of(...items: Slice<T>) -> Self {
        let mut me = Self::new();

        me.push(items);
        me
    }

    @CanBe(i32, u32, type_var = O)
    pub fn len(&self) -> O {
        self.len as O
    }

    @OperandAlias(Operands::ArrayIndex)
    pub fn get(&self, idx: i32 | u32) -> Option<T> {
        let idx = idx as i32;
        
        if (idx >= self.len) {
            return None;
        }

        if (idx < 0) {
            let idx = self.len - idx;

            if (idx >= self.len || idx < 0) {
                return None;
            }

            self.get(idx)
        }

        let pos: u32 = self.base_ptr + (self.get_item_size() * idx as u32)?;
        let item: &[u8] = self.alloc.read::<T>(pos, self.get_item_size())?;

        core::mem::deserialize_struct(item)
    }

    @Overload
    @OperandAlias(Operands::IndexedSet)
    pub fn push(&mut self, item: T, idx: i32 | u32) -> &mut self;

    @OperandAlias(Operands::Add)
    pub fn push(&self, item: Array<T, ?> | Slice<T>) -> &self {
        &*self.clone().push(item)
    }

    @Cast(Slice<T>)
    pub fn as_slice(&self) -> Slice<T> {
        let mut slice = Slice::from_length(self.len);

        for i in 0..self.len {
            slice[i] = self.get(i);
        }

        slice
    }
    
    pub fn push(&mut self, item: T | Array<T, ?> | Slice<T>, idx: Option<i32 | u32>) -> &mut self {
        if (core::types::is_type::<Array<T, ?>>(item) || core::types::is_type::<Array<T, ?>>(item)) {
            let len = self.len;

            for (i, item) in item.as_slice().enumerate() {
                self.push_one(item, idx.or(len) + i);

                return self;
            }
        }

        self.push_one(item, idx)
    }

    fn push_one(&mut self, item: T, idx: Option<i32 | u32>) -> &mut self {
        let idx = idx.or(self.len);

        self.alloc.reserve(&mut self.base_ptr, self.get_item_size());

        let pos = self.base_ptr + (self.get_item_size() * idx);

        self.alloc.push(item, pos);
        self.len++;
        self
    }

    @OperandAlias(Operands::Subtract)
    pub fn pop(&mut self, idx: Option<i32 | u32>) -> &mut self {
        if let Some(idx) = idx {
            let pos = self.base_ptr + (self.get_item_size() * idx);

            self.alloc.pop(pos, self.get_item_size());
            self.len--;
            
            return self;
        }

        let pos = self.base_ptr + (self.get_item_size() * (self.len - 1));

        self.alloc.pop(pos, self.get_item_size());
        self.len--;
        
        self
    }

    const fn get_item_size(&self) -> u32 {
        core::types::size_of::<T>()
    }
}
```
