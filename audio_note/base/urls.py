from django.urls import path, reverse_lazy
from .views import HomeView, RegisterView
from django.contrib.auth.views import LogoutView, LoginView

urlpatterns = [
    path("", HomeView.as_view(), name="home"),
    path("users/register", RegisterView.as_view(), name="register"),
    path(
        "users/login", 
        LoginView.as_view(
            template_name="base/login.html",
            next_page=reverse_lazy('home')
        ), 
        name="login"
    ),
    path(
        "users/logout", 
        LogoutView.as_view(
            next_page=reverse_lazy('home') 
        ), 
        name="logout"
    )
]
