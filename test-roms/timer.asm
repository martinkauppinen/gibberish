SECTION "timer_irq", ROM0[$50]
    jp timer_interrupt

SECTION "Header", ROM0[$100]
    jp start
    ds $150 - @, 0 ; Header

start:
    di              ; Disable interrupts

    ; Enable timer interrupt
    ld hl, $FFFF    ; Interrupt Enable register
    ld a, 4         ; 00100: Timer interrupt bit
    ld [hl], a      ; IE = 0b100
    ei              ; Enable interrupts

    ; Enable timer, 262144 Hz
    ld hl, $FF07    ; TAC
    ld a, 5         ; 101, 1__: Enable, _01: 262144 Hz
    ld [hl], a      ; TAC = 0b101

    ld a, 0         ; Set registers to 0
    ld b, 0

    ; Loop until B != 0. This will happen when TIMA
    ; overflows and triggers the timer interrupt, at which
    ; point the interrupt handler will set B to non-zero.
    loop:
        cp b        ; Compare A and B
        jp z, loop  ; while B == 0 (A), loop

    stop            ; Stop CPU

timer_interrupt:
    ld b, 1         ; B = 1
    reti            ; Return from interrupt
