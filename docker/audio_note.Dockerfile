FROM python:3.12-alpine3.20

COPY ./requirements.txt /tmp/requirements.txt

RUN pip install -r /tmp/requirements.txt

WORKDIR /app

COPY ./ .

CMD [ "python", "manage.py", "runserver", "0.0.0.0:8000"]
