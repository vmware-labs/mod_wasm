<?php

use Twig\Environment;
use Twig\Error\LoaderError;
use Twig\Error\RuntimeError;
use Twig\Extension\SandboxExtension;
use Twig\Markup;
use Twig\Sandbox\SecurityError;
use Twig\Sandbox\SecurityNotAllowedTagError;
use Twig\Sandbox\SecurityNotAllowedFilterError;
use Twig\Sandbox\SecurityNotAllowedFunctionError;
use Twig\Source;
use Twig\Template;

/* @olivero/../images/status.svg */
class __TwigTemplate_77e4f936ab303677eee7cb77f85c6120 extends Template
{
    private $source;
    private $macros = [];

    public function __construct(Environment $env)
    {
        parent::__construct($env);

        $this->source = $this->getSourceContext();

        $this->parent = false;

        $this->blocks = [
        ];
        $this->sandbox = $this->env->getExtension('\Twig\Extension\SandboxExtension');
        $this->checkSecurity();
    }

    protected function doDisplay(array $context, array $blocks = [])
    {
        $macros = $this->macros;
        // line 1
        echo "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"32px\" height=\"32px\" viewBox=\"0 0 32 32\">
  <path d=\"M26.8,12.6c0,0.4-0.1,0.7-0.4,0.9L15.1,24.9c-0.2,0.2-0.6,0.4-1,0.4c-0.3,0-0.7-0.1-0.9-0.4l-7.5-7.5c-0.2-0.2-0.4-0.6-0.4-0.9c0-0.4,0.1-0.7,0.4-1l1.9-1.9c0.2-0.2,0.6-0.4,0.9-0.4c0.4,0,0.7,0.1,0.9,0.4l4.7,4.7l8.5-8.5c0.2-0.2,0.6-0.4,0.9-0.4c0.4,0,0.7,0.1,0.9,0.4l1.9,1.9C26.6,11.9,26.8,12.3,26.8,12.6z M32,16c0-8.8-7.2-16-16-16C7.2,0,0,7.2,0,16c0,8.8,7.2,16,16,16C24.8,32,32,24.8,32,16z\"/>
</svg>
";
    }

    public function getTemplateName()
    {
        return "@olivero/../images/status.svg";
    }

    public function getDebugInfo()
    {
        return array (  39 => 1,);
    }

    public function getSourceContext()
    {
        return new Source("", "@olivero/../images/status.svg", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/olivero/images/status.svg");
    }
    
    public function checkSecurity()
    {
        static $tags = array();
        static $filters = array();
        static $functions = array();

        try {
            $this->sandbox->checkSecurity(
                [],
                [],
                []
            );
        } catch (SecurityError $e) {
            $e->setSourceContext($this->source);

            if ($e instanceof SecurityNotAllowedTagError && isset($tags[$e->getTagName()])) {
                $e->setTemplateLine($tags[$e->getTagName()]);
            } elseif ($e instanceof SecurityNotAllowedFilterError && isset($filters[$e->getFilterName()])) {
                $e->setTemplateLine($filters[$e->getFilterName()]);
            } elseif ($e instanceof SecurityNotAllowedFunctionError && isset($functions[$e->getFunctionName()])) {
                $e->setTemplateLine($functions[$e->getFunctionName()]);
            }

            throw $e;
        }

    }
}
