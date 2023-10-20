Resource: Library Imports
  - async_openai, cargo, ignore, serde, sha2, std

Resource: Plan struct
  Operation: new
    - Initialize a new Plan
  Operation: nodes
    - Get a reference to the nodes in the Plan

Resource: Action enum
  Variant: CodeToRO
    - Convert code element to Resource and Operation
  Variant: FolderToRO
    - Convert folder element to Resource and Operation

Resource: Element struct
  Operation: write_new_self_content
    - Write new self content to the element
  Operation: relative_path
    - Get the relative path of the element
  Operation: find_children
    - Find children elements of the element
  Operation: find_parent
    - Find parent element of the element
  Operation: is_file
    - Check if the element is a file
  Operation: content
    - Get the content of the element

Resource: ActionResult struct
  Field: llm_executed
    - Flag indicating whether the llm (language model) was executed
  Field: llm_input
    - Input for the llm (language model)
  Field: llm_result
    - Result produced by the llm (language model)

Resource: ComputationUnit struct
  Field: action_executed
    - Action executed in the computation unit
  Field: result
    - Result produced by the action execution

Resource: AnalyzeResult struct