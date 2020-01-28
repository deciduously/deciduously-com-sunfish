EXEC=deciduously-com
USER=deciduously0
VERSION=latest
TAG=08fd6725d5d5
PORT=3000
REMOTE=$(USER)/$(EXEC)

docker:
	docker build -t $(EXEC) .

deploy:
	docker tag $(TAG) $(REMOTE):$(VERSION)
	docker push $(REMOTE)

run:
	docker run -dit -p $(PORT):8080 $(EXEC)