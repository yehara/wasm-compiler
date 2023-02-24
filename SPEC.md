## 利用している命令

### header
 - `0x00 0x61 0x73 0x6d` wasm binary magic
 - `0x01 0x00 0x00 0x00` wasm binary version

### section code
 - `0x01` type
 - `0x03` function
 - `0x07` export
 - `0x0a` code

### type section
function 毎に別の type を定義。params は function の定義に従う。型は i32 のみ。result は単一の i32 で固定
- `0x60` function type
- `(num params) 0x7f...` num params + パラメータ数の i32 
- `0x01 0x7f` num results + i32 

### function section
function の index は定義順。type の index と一致。

### export section
固定で main 関数を 1 つのみを export する。
- `0x01` num exports
- `0x04 0x6d 0x61 0x69 0x6e`
- `0x00` export kind
- `(func_idx)` main 関数の index

### code section
- `0x02 0x40` block
- `0x03 0x40` loop
- `0x04 0x40` if
- `0x05` else
- `0x0b` end
- `0x0c (block_idx)` br (block_idx)
- `0x0d (block_idx)` br_if (block_idx)
- `0x0f` return
- `0x10 (func_idx)` call (func_idx)
- `0x1a` drop
- `0x20 (local_idx)` local.get (local_idx)
- `0x22 (local_idx)` local.tee (local_idx)
- `0x41 (LEB128)` i32.const (num)
- `0x46` i32.eq
- `0x47` i32.ne
- `0x48` i32.lt_s
- `0x4a` i32.gt_s
- `0x4e` i32.ge_s
- `0x4c` i32.le_s
- `0x6a` i32.add
- `0x6b` i32.sub
- `0x6c` i32.mul
- `0x6d` i32.div_s