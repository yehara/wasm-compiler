(module
  (func $main
    (param $num i32)
    (result i32)
    local.get $num
    call $fib2
    return
    i32.const 0
  )
  (func $fib2
    (param $a i32)
    (result i32)
    (local $p0 i32)
    (local $p1 i32)
    (local $i i32)
    (local $p2 i32)
    local.get $a
    i32.const 1
    i32.le_s
    (if
      (then
    local.get $a
    return
      )
    )
    i32.const 0
    local.tee $p0
    drop
    i32.const 1
    local.tee $p1
    drop
    i32.const 2
    local.tee $i
    drop
    (block $block41
      (loop $loop41
    local.get $i
    local.get $a
    i32.le_s
        i32.const 0
        i32.eq
        br_if $block41
    local.get $p0
    local.get $p1
    i32.add
    local.tee $p2
    drop
    local.get $p1
    local.tee $p0
    drop
    local.get $p2
    local.tee $p1
    drop
    local.get $i
    i32.const 1
    i32.add
    local.tee $i
        drop
        br $loop41
      )
    )
    local.get $p2
    return
    i32.const 0
  )
  (export "main" (func $main))
)
