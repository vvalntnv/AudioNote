from typing import TYPE_CHECKING, Any
from django.contrib.auth.signals import user_logged_in, user_login_failed
from django.dispatch import receiver
from django.contrib import messages
from django.http.request import HttpRequest

if TYPE_CHECKING:
    from django.contrib.auth.models import User


@receiver(user_logged_in)
def on_user_logged_in(sender: Any, request: HttpRequest, user: "User", **kwargs) -> None:
    _ = sender, kwargs
    messages.success(request, f"{user.username} has logged in successfully")

@receiver(user_login_failed)
def on_user_login_failed(sender: Any, credentials: dict, request: HttpRequest, **kwargs) -> None:
    _ = sender, kwargs
    messages.error(request, f"Login failed for {credentials['username']}")
