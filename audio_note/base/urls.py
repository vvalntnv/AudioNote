from django.urls import path

from base.views import HomeView, RegisterView

urlpatterns = [
    path("", HomeView.as_view(), name="home"),
    path("register", RegisterView.as_view(), name="register")
]
