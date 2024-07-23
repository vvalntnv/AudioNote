from django.shortcuts import render
from django.urls import reverse_lazy
from django.views.generic import CreateView, TemplateView
from .forms import RegisterForm

# Create your views here.
class HomeView(TemplateView):
    template_name = "base/home.html"

class RegisterView(CreateView):
    form_class = RegisterForm
    template_name = 'base/register.html'
    success_url = reverse_lazy("home")
