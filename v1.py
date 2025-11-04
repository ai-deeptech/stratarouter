import stratarouter
print(f"✓ Version: {stratarouter.__version__}")

from stratarouter import Route, RouteLayer
print("✓ Classes imported")

from stratarouter.encoders import HuggingFaceEncoder
print("✓ Encoders available")

print("\n✨ All set!")
