SECTION "Header", ROM0[$100]
    jp start
    ds $150 - @, 0 ; Header

start:
    di              ; Disable interrupts

    ; Enable V-Blank interrupt
    ld hl, $FFFF
    ld a, 1
    ld [hl], a
                
    ; Request V-Blank interrupt
    ld hl, $FF0F
    ld [hl], a

    xor a, a        ; Set A to 0
    halt            ; Cause halt bug
    inc a           ; Increment A, this will happen twice!
    
    ld hl, $0040
    ld a, $D3       ; Invalid opcode
    ld [hl], a
    ei              ; This will crash the emulator
