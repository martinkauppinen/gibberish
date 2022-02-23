SECTION "Header", ROM0[$100]
    jp start
    ds $150 - @, 0 ; Header

start:
    ld de, $dead ; Values to check that stack
    ld hl, $beef ; popping works as intended

    ld a, $0
    ld b, $1
    call multiply

    ld a, $2
    ld b, $2
    call multiply

    ld a, $5
    ld b, $3
    call multiply

    ld a, $F
    ld b, $F
    call multiply

    di
    ; Enable V-Blank interrupt
    ld hl, $FFFF
    ld a, 1
    ld [hl], a
                
    ; Request V-Blank interrupt
    ld hl, $FF0F
    ld [hl], a
    ld hl, $0040
    ld a, $D3       ; Invalid opcode
    ld [hl], a
    ei              ; This will crash the emulator
    nop
    
; Multiply A with B through repeated addition
; Result is stored in A
multiply:

    ; Early quit if one factor is zero
    push hl
    ld h, $0
    cp h
    pop hl
    ret z

    call swap_ab

    push hl
    ld h, $0
    cp h
    pop hl
    ret z

    push de
    ld d, a
    ld a, $0

    loop:
        add a, b
        dec d
        jp nz, loop
    pop de

    ret

; Swap contents of registers A and B
swap_ab:
    push de
    ld d, a
    ld a, b
    ld b, d
    pop de
    ret
