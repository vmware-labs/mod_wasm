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

/* core/themes/claro/templates/install-page.html.twig */
class __TwigTemplate_5c82c8c2d1b1721a1284c5c4ab17104c extends Template
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
        // line 12
        echo "<div class=\"layout-container\">

  <header role=\"banner\">
    ";
        // line 15
        if (($context["site_name"] ?? null)) {
            // line 16
            echo "      <h1 class=\"site-name\">
        ";
            // line 17
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["site_name"] ?? null), 17, $this->source), "html", null, true);
            echo "
        ";
            // line 18
            if (($context["site_version"] ?? null)) {
                // line 19
                echo "          <span class=\"site-version\">";
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["site_version"] ?? null), 19, $this->source), "html", null, true);
                echo "</span>
        ";
            }
            // line 21
            echo "      </h1>
    ";
        }
        // line 23
        echo "  </header>

  ";
        // line 25
        if (twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "sidebar_first", [], "any", false, false, true, 25)) {
            // line 26
            echo "    <aside class=\"layout-sidebar-first\" role=\"complementary\">
      ";
            // line 27
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "sidebar_first", [], "any", false, false, true, 27), 27, $this->source), "html", null, true);
            echo "
    </aside>";
            // line 29
            echo "  ";
        }
        // line 30
        echo "
  <main role=\"main\" class=\"main-content\">
    ";
        // line 32
        if (($context["title"] ?? null)) {
            // line 33
            echo "      <h2 class=\"heading-c\">";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["title"] ?? null), 33, $this->source), "html", null, true);
            echo "</h2>
    ";
        }
        // line 35
        echo "    ";
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "highlighted", [], "any", false, false, true, 35), 35, $this->source), "html", null, true);
        echo "
    ";
        // line 36
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "content", [], "any", false, false, true, 36), 36, $this->source), "html", null, true);
        echo "
  </main>

  ";
        // line 39
        if (twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "sidebar_second", [], "any", false, false, true, 39)) {
            // line 40
            echo "    <aside class=\"layout-sidebar-second\" role=\"complementary\">
      ";
            // line 41
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "sidebar_second", [], "any", false, false, true, 41), 41, $this->source), "html", null, true);
            echo "
    </aside>";
            // line 43
            echo "  ";
        }
        // line 44
        echo "
  ";
        // line 45
        if (twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "page_bottom", [], "any", false, false, true, 45)) {
            // line 46
            echo "    <footer role=\"contentinfo\">
      ";
            // line 47
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["page"] ?? null), "page_bottom", [], "any", false, false, true, 47), 47, $this->source), "html", null, true);
            echo "
    </footer>
  ";
        }
        // line 50
        echo "
</div>";
    }

    public function getTemplateName()
    {
        return "core/themes/claro/templates/install-page.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  130 => 50,  124 => 47,  121 => 46,  119 => 45,  116 => 44,  113 => 43,  109 => 41,  106 => 40,  104 => 39,  98 => 36,  93 => 35,  87 => 33,  85 => 32,  81 => 30,  78 => 29,  74 => 27,  71 => 26,  69 => 25,  65 => 23,  61 => 21,  55 => 19,  53 => 18,  49 => 17,  46 => 16,  44 => 15,  39 => 12,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/themes/claro/templates/install-page.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/claro/templates/install-page.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("if" => 15);
        static $filters = array("escape" => 17);
        static $functions = array();

        try {
            $this->sandbox->checkSecurity(
                ['if'],
                ['escape'],
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
