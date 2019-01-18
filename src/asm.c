#include <stdint.h>
#include <stdio.h>

#include "dasm_proto.h"
#include "dasm_x86.h"

|.arch x64
|.section code
|.globals lbl_
|.actionlist bf_actions

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
  |.code
  return DASM_S_OK;
}

int asm_emit_prologue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  | push rbp
  | mov rbp, rdi
  return DASM_S_OK;
}

int asm_emit_epilogue(AsmContext* context)
{
  dasm_State** Dst = &context->d;
  | pop rbp
  | ret
  return DASM_S_OK;
}

int asm_mov(AsmContext* context, unsigned reg, uint64_t value)
{
  dasm_State** Dst = &context->d;
  /* TODO: handle the case when value is larger than 32 bit */
  | mov qword [rbp+reg*8], value
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
