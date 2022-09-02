(module
  (func $main
    (result i32)
    i32.const 10
    call $fib1
    return
    drop
    i32.const 0
  )
  (func $fib1
    (param $a i32)
    (result i32)
    local.get $a
    i32.const 1
    i32.le_s
    (if
      (then
    local.get $a
    return
    drop
    i32.const 0
      drop
      )
    )
    i32.const 0
    drop
    local.get $a
    i32.const 2
    i32.sub
    call $fib1
    local.get $a
    i32.const 1
    i32.sub
    call $fib1
    i32.add
    return
    drop
    i32.const 0
  )
  (export "main" (func $main))
)
