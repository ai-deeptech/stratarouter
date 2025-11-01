"""
#!/bin/bash
set -e

echo "Verifying symbol stripping..."

FAILED=0

for wheel in target/wheels/*.whl; do
    if [ -f "$wheel" ]; then
        echo "Checking $wheel..."
        unzip -q "$wheel" -d temp_wheel
        
        for so in temp_wheel/**/*.so; do
            if [ -f "$so" ]; then
                SYMBOLS=$(nm -D "$so" 2>/dev/null | wc -l)
                echo "  $so: $SYMBOLS symbols"
                if [ "$SYMBOLS" -gt 100 ]; then
                    echo "  ⚠️  WARNING: Too many symbols"
                    FAILED=1
                fi
            fi
        done
        
        rm -rf temp_wheel
    fi
done

if [ $FAILED -eq 0 ]; then
    echo ""
    echo "✅ All binaries properly stripped!"
else
    echo ""
    echo "❌ Some binaries have too many symbols"
    exit 1
fi
"""