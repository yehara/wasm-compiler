(module
  (func $main
    (param $a i32)
    (param $b i32)
    (result i32)
    local.get $a
    local.get $b
    call $lcm
    return
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
      )
    )
    local.get $a
    local.get $b
    i32.eq
    (if
      (then
    local.get $a
    return
      )
    )
    local.get $b
    i32.const 0
    i32.eq
    (if
      (then
    local.get $a
    return
      )
    )
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
    i32.const 0
  )
  (export "main" (func $main))
)
