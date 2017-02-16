        SETL    0, 0, 0
        SETL    1, 0, 113
        SETL    2, 0, 2
LOOP:   CMPU    4, 1, 2
        BZ      4, 0, PRIME
        DIVU    4, 1, 2
        GET     3, 0, 6
        BZ      3, 0, END
        INCL    2, 0, 1
        JMPB    0, 0, LOOP
PRIME:  SETL    0, 0, 1
END:    TRAP    0, 0, 0
