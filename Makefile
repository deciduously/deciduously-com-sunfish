EXEC=deciduously-com
USER=deciduously0
VERSION=latest
TAG=08fd6725d5d5
REMOTE=$(USER)/$(EXEC)

docker:
	docker build -t $(EXEC) .

deploy:
	docker tag $(TAG) $(REMOTE):$(VERSION)
	docker push $(REMOTE)