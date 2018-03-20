MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  FLASH : ORIGIN = 0x00000000, LENGTH = 0x00080000
  RAM : ORIGIN = 0x1fff8000, LENGTH = 128K

  /* This chip has funny security bits at 0x400 ... */
  FCF : ORIGIN = 0x00000400, LENGTH = 0x00000010
}

/* This is where the call stack will be allocated. */
/* The stack is of the full descending type. */
/* NOTE Do NOT modify `_stack_start` unless you know what you are doing */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

/* Skip the whole sector with the security bits and let program code start
 * after to prevent nasty mishaps */
_stext = 0x00000800;
