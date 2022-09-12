build-IntellidynCloudFunction:
	@echo "compiling...."
	cargo build --release --target x86_64-unknown-linux-musl

	@echo "creating intellidyn bootstrap zip file..."
	cp ./target/x86_64-unknown-linux-musl/release/intellidyn ./bootstrap
	zip -r bootstrap.zip bootstrap

	@echo "uploading to s3 bucket..."
	aws s3 cp bootstrap.zip s3://intellidyn-inc-bucket

	@echo "create lambda function..."
	aws lambda create-function \
	--function-name Intellidyn3 \
	--runtime provided.al2 \
	--handler main.handler_func_closure \
	--role arn:aws:iam::105390037103:role/Intellidyn_Inc_Policy \
	--zip-file fileb://bootstrap.zip