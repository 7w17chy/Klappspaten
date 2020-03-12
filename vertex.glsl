#version 300 ES core

layout(location = 0) in vec4 postion;

void main()
{
   gl_Position = postion;
}
