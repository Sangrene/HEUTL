import argparse
from RestrictedPython import compile_restricted, Eval, Guards
import json

def main(script: str, input: str):
  byte_code = compile_restricted(
      script,
      "<string>",
      "exec"
  )
  def get_item(ob, key):
    return ob[key]
  
  restricted_globals = {
    **Guards.safe_builtins,
    "_getiter_": Eval.default_guarded_getiter,
    "_iter_unpack_sequence_": Guards.guarded_iter_unpack_sequence,
    "_getattr_": Guards.safer_getattr,
    "_getitem_": get_item,
    "input": json.loads(input),
    "result": None
  }
  
  exec(byte_code, restricted_globals)
  print(json.dumps(restricted_globals["result"]))
    
    
  
  
if __name__ == "__main__":
  parser=argparse.ArgumentParser()

  parser.add_argument("--script", help="Script to run", type=str)
  parser.add_argument("--input", help="Input as json string", type=str)
  
  args=parser.parse_args()
  main(args.script, args.input)