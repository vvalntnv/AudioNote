from django.apps import AppConfig
from django.contrib.auth import user_logged_in

from base.signals import on_user_logged_in


class BaseConfig(AppConfig):
    default_auto_field = 'django.db.models.BigAutoField'
    name = 'base'

    def ready(self):
        user_logged_in.connect(on_user_logged_in)
        return super().ready()
