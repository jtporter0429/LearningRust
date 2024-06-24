### Scalars represent individual values - Integers, floating-point numbers, Booleans, and characters

## Integers do not have a fractional component - whole numbers only. They can be signed or unsigned
## Floating-point numbers have decimal values attached to them
## Boolean types are either true or false - may be represented by 1 and 0
## Character types (char) are characters of any kind - lowercase, uppercase, and emojis are all included

### Compound Types

## Tuples
# - have a fixed length and cannot grow or shrink in size once declared (let tup: (i32, f64, u8) = (500, 6.4, 1), followed by let (x, y, z) = tup assigns values to variables in the same order)
# - values can be access directly by using a period followed by the index. Using the previous tuple, we can ccess using let five_hundred = x.0

## Arrays
# - Not as flexible as a vector, but more useful when data is to be on the stack rather than the heap or when the number of elements will not need to change
# - use square brackets with type of element, then semicolon, then number of elements: let a: [i32; 5] = [1, 2, 3, 4, 5];
# - let a = [3; 5] creates 5 elements with a value of 3
#            ^  ^
#          value
#            num of elem.