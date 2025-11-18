cd ./state-processor/
source ./maturin-venv/bin/activate
maturin build
# assumes build succeeds
deactivate
cd ../game-renderer/
source venv/bin/activate 
cd ../state-processor/target/wheels
pip install --force-reinstall *.whl
cd ../../../game-renderer/
deactivate
