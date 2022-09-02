(module
  (func $main
    (result i32)
    i32.const 12
    i32.const 20
    call $lcm
    return
    drop
    i32.const 0
  )
  (func $lcm
    (param $a i32)
    (param $b i32)
    (result i32)
    local.get $a
    local.get $a
    local.get $b
    call $gcd
    i32.div_s
    local.get $b
    i32.mul
    return
    drop
    i32.const 0
  )
  (func $gcd
    (param $a i32)
    (param $b i32)
    (result i32)
    local.get $a
    local.get $b
    i32.lt_s
    (if
      (then
    local.get $b
    local.get $a
    call $gcd
    return
    drop
    i32.const 0
      drop
      )
    )
    i32.const 0
    drop
    local.get $a
    local.get $b
    i32.eq
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
    local.get $b
    i32.const 0
    i32.eq
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
    local.get $b
    local.get $a
    local.get $a
    local.get $b
    i32.div_s
    local.get $b
    i32.mul
    i32.sub
    call $gcd
    return
    drop
    i32.const 0
  )
  (export "main" (func $main))
)
