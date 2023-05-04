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

/* @olivero/../images/rss.svg */
class __TwigTemplate_a07105e19dfd802ddd0c131811136e4e extends Template
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
        echo "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"14.2\" height=\"14.2\" viewBox=\"0 0 14.2 14.2\">
  <path d=\"M4,12.2c0-2.5-3.9-2.4-3.9,0C0.1,14.7,4,14.6,4,12.2z M9.1,13.4C8.7,9,5.2,5.5,0.8,5.1c-1,0-1,2.7-0.1,2.7c3.1,0.3,5.5,2.7,5.8,5.8c0,0.7,2.1,0.7,2.5,0.3C9.1,13.7,9.1,13.6,9.1,13.4z M14.2,13.5c-0.1-3.5-1.6-6.9-4.1-9.3C7.6,1.7,4.3,0.2,0.8,0c-1,0-1,2.6-0.1,2.6c5.8,0.3,10.5,5,10.8,10.8C11.5,14.5,14.3,14.4,14.2,13.5z\"/>
</svg>
";
    }

    public function getTemplateName()
    {
        return "@olivero/../images/rss.svg";
    }

    public function getDebugInfo()
    {
        return array (  39 => 1,);
    }

    public function getSourceContext()
    {
        return new Source("", "@olivero/../images/rss.svg", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/olivero/images/rss.svg");
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
