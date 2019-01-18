/*
** This file has been pre-processed with DynASM.
** http://luajit.org/dynasm.html
** DynASM version 1.4.0, DynASM x64 version 1.4.0
** DO NOT EDIT! The original file is in "src/asm.c".
*/

#line 1 "src/asm.c"
#include <stdint.h>
#include <stdio.h>

#include "dasm_proto.h"
#include "dasm_x86.h"

//|.arch x64
#if DASM_VERSION != 10400
#error "Version mismatch between DynASM and included encoding engine"
#endif
#line 8 "src/asm.c"
//|.section code
#define DASM_SECTION_CODE	0
#define DASM_MAXSECTION		1
#line 9 "src/asm.c"
//|.globals lbl_
enum {
  lbl__MAX
};
#line 10 "src/asm.c"
//|.actionlist bf_actions
static const unsigned char bf_actions[17] = {
  254,0,85,72,137,252,253,255,93,195,255,72,199,133,233,237,255
};

#line 11 "src/asm.c"

typedef struct {
  dasm_State* d;
  void* labels[lbl__MAX];
  unsigned npc;
  unsigned nextpc;
} AsmContext;

AsmContext* asm_new()
{
  AsmContext* context = malloc(sizeof(AsmContext));
  dasm_init(&context->d, DASM_MAXSECTION);
  dasm_setupglobal(&context->d, context->labels, lbl__MAX);
  context->npc = 8;
  context->nextpc = 0;
  return context;
}

void asm_finalize(AsmContext* context)
{
  dasm_free(&context->d);
  free(context);
}

int asm_setup(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  dasm_setup(&context->d, bf_actions);
  dasm_growpc(&context->d, context->npc);
  //|.code
  dasm_put(Dst, 0);
#line 41 "src/asm.c"
  return DASM_S_OK;
}

int asm_emit_prologue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  //| push rbp
  //| mov rbp, rdi
  dasm_put(Dst, 2);
#line 49 "src/asm.c"
  return DASM_S_OK;
}

int asm_emit_epilogue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  //| pop rbp
  //| ret
  dasm_put(Dst, 8);
#line 57 "src/asm.c"
  return DASM_S_OK;
}

int asm_mov(AsmContext* context, unsigned reg, uint64_t value)
{
  dasm_State** Dst = &context->d;
  /* TODO: handle the case when value is larger than 32 bit */
  //| mov qword [rbp+reg*8], value
  dasm_put(Dst, 11, reg*8, value);
#line 65 "src/asm.c"
  return DASM_S_OK;
}

int asm_link(AsmContext* context, size_t *szp)
{
  return dasm_link(&context->d, szp);
}

int asm_encode(AsmContext* context, void *buffer)
{
  return dasm_encode(&context->d, buffer);
}
