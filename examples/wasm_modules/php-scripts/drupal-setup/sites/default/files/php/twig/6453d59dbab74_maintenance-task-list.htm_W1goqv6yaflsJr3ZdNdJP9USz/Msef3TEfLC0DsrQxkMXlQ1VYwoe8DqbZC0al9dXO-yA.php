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

/* core/modules/system/templates/maintenance-task-list.html.twig */
class __TwigTemplate_b69ad0e803c38c6a388c004c8611141a extends Template
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
        // line 17
        echo "<h2 class=\"visually-hidden\">";
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(t("Installation tasks"));
        echo "</h2>
<ol class=\"task-list\">
";
        // line 19
        $context['_parent'] = $context;
        $context['_seq'] = twig_ensure_traversable(($context["tasks"] ?? null));
        foreach ($context['_seq'] as $context["_key"] => $context["task"]) {
            // line 20
            echo "  <li";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["task"], "attributes", [], "any", false, false, true, 20), 20, $this->source), "html", null, true);
            echo ">
    ";
            // line 21
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["task"], "item", [], "any", false, false, true, 21), 21, $this->source), "html", null, true);
            echo "
    ";
            // line 22
            if (twig_get_attribute($this->env, $this->source, $context["task"], "status", [], "any", false, false, true, 22)) {
                echo "<span class=\"visually-hidden\"> (";
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["task"], "status", [], "any", false, false, true, 22), 22, $this->source), "html", null, true);
                echo ")</span>";
            }
            // line 23
            echo "  </li>
";
        }
        $_parent = $context['_parent'];
        unset($context['_seq'], $context['_iterated'], $context['_key'], $context['task'], $context['_parent'], $context['loop']);
        $context = array_intersect_key($context, $_parent) + $_parent;
        // line 25
        echo "</ol>
";
    }

    public function getTemplateName()
    {
        return "core/modules/system/templates/maintenance-task-list.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  71 => 25,  64 => 23,  58 => 22,  54 => 21,  49 => 20,  45 => 19,  39 => 17,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/modules/system/templates/maintenance-task-list.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/modules/system/templates/maintenance-task-list.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("for" => 19, "if" => 22);
        static $filters = array("t" => 17, "escape" => 20);
        static $functions = array();

        try {
            $this->sandbox->checkSecurity(
                ['for', 'if'],
                ['t', 'escape'],
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
