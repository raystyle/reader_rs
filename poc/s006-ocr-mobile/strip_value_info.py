# /// script
# requires-python = ">=3.10"
# dependencies = ["onnx"]
# ///
"""剥离 ONNX 模型的中间 value_info 静态形状元数据（tract 符号维度统一冲突规避）。"""
import sys
import onnx

src, dst = sys.argv[1], sys.argv[2]
m = onnx.load(src)
n = len(m.graph.value_info)
del m.graph.value_info[:]
# 输出 shape 声明也可能是静态的，一并清为动态
for out in m.graph.output:
    for d in out.type.tensor_type.shape.dim:
        d.ClearField("dim_value")
        d.dim_param = "dyn"
onnx.save(m, dst)
print(f"{src}: stripped {n} value_info entries -> {dst}")
